using System;
using System.IO;
using System.Runtime.CompilerServices;
using System.Text;

namespace ModLoader {
    static class Native {
        static bool available = true;

        [MethodImpl(MethodImplOptions.InternalCall)]
        static extern void Log(int level, string source, string message);

        [MethodImpl(MethodImplOptions.NoInlining)]
        static void Call(int level, string source, string message) => Log(level, source, message);

        internal static bool Available => available;

        internal static bool TryLog(LogLevel level, string source, string message) {
            if (!available) {
                return false;
            }
            try {
                Call((int)level, source, message);
                return true;
            } catch (Exception) {
                available = false;
                return false;
            }
        }
    }

    sealed class ConsoleRedirect : TextWriter {
        readonly Logger logger;
        readonly StringBuilder line = new StringBuilder();

        public ConsoleRedirect(Logger logger) {
            this.logger = logger;
        }

        public override Encoding Encoding => Encoding.UTF8;

        public override void Write(char value) {
            lock (line) {
                if (value == '\n') {
                    Flush();
                } else if (value != '\r') {
                    line.Append(value);
                }
            }
        }

        public override void Write(string value) {
            if (value == null) {
                return;
            }
            foreach (var c in value) {
                Write(c);
            }
        }

        public override void Flush() {
            lock (line) {
                if (line.Length == 0) {
                    return;
                }
                var text = line.ToString();
                line.Length = 0;
                logger.Info(text);
            }
        }
    }
}
