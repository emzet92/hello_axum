// assembly/index.ts

@external("env", "get_temp")
declare function get_temp(): f64;

@external("env", "get_pressure")
declare function get_pressure(): f64;

export function evaluate(): bool {
  return get_temp() > 30 && get_pressure() < 10;
}

@external("env", "log")
declare function log(): void;

export function run(): void {
  log();
}