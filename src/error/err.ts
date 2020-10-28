import { Position } from "./position.ts";

export class Err {
  public constructor(
    public name: string,
    public positionStart: Position,
    public positionEnd: Position,
    public description: string,
  ) {}

  public formatted(): string {
    return `File Name: ${this.positionStart.fileName}\nLine: ${this
      .positionStart.line +
      1} \nStarts At: ${this.positionStart.index +
      1}\nEnds At: ${this.positionEnd.index +
      1}\nName: ${this.name}\nDescription: ${this.description}`;
  }
}
