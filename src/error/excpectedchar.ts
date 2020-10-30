import { Err } from "./err.ts";
import { Position } from "./position.ts";

export class ExcpectedChareterError extends Err {
  constructor(start: Position, end: Position, description: string) {
    super("Excpected Chareter", start, end, description);
  }
}
