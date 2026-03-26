@external("env", "get_temp")
declare function get_temp(): f64;

@external("env", "get_pressure")
declare function get_pressure(): f64;

@external("env", "log")
declare function log(): void;

// 🔥 eksportujesz ręcznie
export { get_temp, get_pressure, log};