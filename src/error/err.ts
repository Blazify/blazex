import { Context } from "../core/context.ts";
import { Position } from "./position.ts";

export class Err {
  public constructor(
    public name: string,
    public positionStart: Position,
    public positionEnd: Position,
    public description: string,
  ) {}

  public formatted(): string {
    return `${this.name}: ${this.description}\nFile ${this.positionStart.fileName}, line ${this.positionStart.line + 1}`;
  }
}