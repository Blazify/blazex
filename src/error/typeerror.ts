import { Err } from "./err.ts";
import { Position } from "./position.ts";

export class InvalidTypeError extends Err {
  constructor(start: Position, end: Position, description: string) {
    super("Type Error", start, end, description);
  }
}
