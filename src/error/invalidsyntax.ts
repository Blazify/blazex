import { Err } from "./err.ts";
import { Position } from "./position.ts";

export class InvalidSyntaxError extends Err {
  constructor(start: Position, end: Position, description: string) {
    super("Invalid Syntax", start, end, description);
  }
}
