import { Err } from "./err.ts";
import { Position } from "./position.ts";

export class RuntimeError extends Err {
  constructor(start: Position, end: Position, description: string) {
    super("Runtime Error", start, end, description);
  }
}
