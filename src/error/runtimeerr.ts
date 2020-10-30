import { Context } from "../core/context.ts";
import { Err } from "./err.ts";
import { Position } from "./position.ts";

export class RuntimeError extends Err {
  constructor(
    start: Position,
    end: Position,
    description: string,
    public context: Context,
  ) {
    super("Runtime Error", start, end, description);
  }

  public formatted(): string {
    let res = this.generateTraceback();
    res += `${this.name}: ${this.description}`;
    return res;
  }

  public generateTraceback(): string {
    let res = "";
    let position = this.positionStart;
    let ctx = this.context;
    while (ctx) {
      res = `File ${
        position
          ? position.fileName
          : "Unknown"
      }, line: ${
        position
          ? position.line +
            1
          : "Unknown"
      }, in ${ctx.displayName}\n` + res;
      if (ctx.parentEntryPosition) position = ctx.parentEntryPosition!;
      if (ctx) ctx = ctx.parent!;
    }

    return `Traceback (most recent call last):\n${res}`;
  }
}
