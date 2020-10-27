import { Err } from "./err.ts";
import { Position } from "./position.ts";

export class IllegalCharecterError extends Err {
  constructor(start: Position, end: Position, description: string) {
    super("Illegal Charecter Error", start, end, description);
  }
}
