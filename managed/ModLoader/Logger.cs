using System;
using System.IO;
using System.Text;

namespace ModLoader {
    public enum LogLevel {
        Debug,
        Info,
        Warning,
        Error
    }

    public sealed class Logger {
        static readonly object sync = new object();

        public string Source { get; }

        public Logger(string source) {
            Source = source;
        }

        public void Debug(object msg) => Write(LogLevel.Debug, msg);
        public void Info(object msg) => Write(LogLevel.Info, msg);
        public void Warning(object msg) => Write(LogLevel.Warning, msg);
        public void Error(object msg) => Write(LogLevel.Error, msg);

        public void Write(LogLevel level, object msg) {
            var text = msg?.ToString() ?? "null";
            if (Native.TryLog(level, Source, text)) {
                return;
            }
            var line = $"[{DateTime.Now:HH:mm:ss.fff}] [{Source}] [{level}] {text}";
            lock (sync) {
                try {
                    if (Paths.Log != null) {
                        File.AppendAllText(Paths.Log, line + "\r\n", Encoding.UTF8);
                    }
                } catch {
                }
            }
        }
    }
}
