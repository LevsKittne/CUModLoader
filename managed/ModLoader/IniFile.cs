using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

namespace ModLoader {
    public sealed class IniFile {
        readonly Dictionary<string, Dictionary<string, string>> sections = new Dictionary<string, Dictionary<string, string>>(StringComparer.OrdinalIgnoreCase);

        public string Path { get; }

        public IniFile(string path) {
            Path = path;
            var text = ReadShared(path);
            if (text != null) {
                Parse(text.Split(new[] { "\r\n", "\n" }, StringSplitOptions.None));
            }
        }

        void Parse(string[] lines) {
            var section = "";
            foreach (var raw in lines) {
                var line = raw.Trim();
                if (line.Length == 0 || line[0] == ';' || line[0] == '#') {
                    continue;
                }
                if (line[0] == '[' && line[line.Length - 1] == ']') {
                    section = line.Substring(1, line.Length - 2).Trim();
                    continue;
                }
                var eq = line.IndexOf('=');
                if (eq > 0) {
                    Set(section, line.Substring(0, eq).Trim(), line.Substring(eq + 1).Trim());
                }
            }
        }

        public string Get(string section, string key, string def = "") {
            return sections.TryGetValue(section, out var s) && s.TryGetValue(key, out var v) ? v : def;
        }

        public bool GetBool(string section, string key, bool def) {
            var v = Get(section, key, null);
            if (v == null) {
                return def;
            }
            v = v.ToLowerInvariant();
            return v == "true" || v == "1" || v == "yes";
        }

        public float GetFloat(string section, string key, float def) {
            return float.TryParse(Get(section, key, null), System.Globalization.NumberStyles.Float, System.Globalization.CultureInfo.InvariantCulture, out var f) ? f : def;
        }

        public string[] GetList(string section, string key) {
            return Get(section, key).Split(',').Select(x => x.Trim()).Where(x => x.Length > 0).ToArray();
        }

        public void Set(string section, string key, string value) {
            if (!sections.TryGetValue(section, out var s)) {
                s = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
                sections[section] = s;
            }
            s[key] = value;
        }

        public bool Save() {
            var sb = new StringBuilder();
            foreach (var s in sections) {
                if (s.Key.Length > 0) {
                    sb.Append('[').Append(s.Key).Append("]\r\n");
                }
                foreach (var kv in s.Value) {
                    sb.Append(kv.Key).Append(" = ").Append(kv.Value).Append("\r\n");
                }
                sb.Append("\r\n");
            }
            var content = sb.ToString();
            if (ReadShared(Path) == content) {
                return true;
            }
            var tmp = Path + "." + System.Diagnostics.Process.GetCurrentProcess().Id + ".tmp";
            for (var attempt = 0; attempt < 5; attempt++) {
                try {
                    File.WriteAllText(tmp, content);
                    if (File.Exists(Path)) {
                        File.Replace(tmp, Path, null);
                    } else {
                        File.Move(tmp, Path);
                    }
                    return true;
                } catch (IOException) {
                    System.Threading.Thread.Sleep(20 * (attempt + 1));
                } catch (UnauthorizedAccessException) {
                    System.Threading.Thread.Sleep(20 * (attempt + 1));
                }
            }
            try {
                File.Delete(tmp);
            } catch {
            }
            return false;
        }

        static string ReadShared(string path) {
            for (var attempt = 0; attempt < 5; attempt++) {
                try {
                    if (!File.Exists(path)) {
                        return null;
                    }
                    using (var fs = new FileStream(path, FileMode.Open, FileAccess.Read, FileShare.ReadWrite | FileShare.Delete))
                    using (var r = new StreamReader(fs)) {
                        return r.ReadToEnd();
                    }
                } catch (IOException) {
                    System.Threading.Thread.Sleep(20 * (attempt + 1));
                }
            }
            return null;
        }
    }
}
