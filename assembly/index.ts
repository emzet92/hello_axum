@external("env", "get_temp")
declare function get_temp(): f64;

@external("env", "get_pressure")
declare function get_pressure(): f64;

@external("env", "log_utf16")
declare function log_utf16(ptr: usize, len: i32): void;

@external("env", "get_value_utf16")
declare function get_value_utf16(ptr: usize, len: i32): f64;

function getValue(path: string): f64 {
  return get_value_utf16(changetype<usize>(path), path.length);
}

function logString(message: string): void {
  log_utf16(changetype<usize>(message), message.length);
}

export function evaluate(): bool {
  let temp = getValue("Motor1/Sensor/Temp");
  let pressure = getValue("Motor1/Sensor/Pressure");

  logString("kupa xd");

  return temp > 30 && pressure < 10;
}

export function run(): void {
  logString("Hello z AssemblyScript!");
  logString("evaluate() zaraz poleci");

  let temp = getValue("Motor1/Sensor/Temp");

  logString("Temp = " + temp.toString());

}