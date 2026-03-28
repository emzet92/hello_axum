@external("env", "get_temp")
declare function get_temp(): f64;

@external("env", "get_pressure")
declare function get_pressure(): f64;

@external("env", "log_utf16")
declare function log_utf16(ptr: usize, len: i32): void;

function logString(message: string): void {
  log_utf16(changetype<usize>(message), message.length);
}

export function evaluate(): bool {
  return get_temp() > 30 && get_pressure() < 10;
}

export function run(): void {
  logString("Hello z AssemblyScript!");
  logString("evaluate() zaraz poleci");
}