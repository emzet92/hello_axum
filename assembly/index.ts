// assembly/index.ts
import { get_temp, get_pressure, log } from "./env";

export function evaluate(): bool {
  return get_temp() > 30 && get_pressure() < 10;
}


export function run(): void {
  log();
}