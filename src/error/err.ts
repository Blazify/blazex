import { Position } from "./position.ts";

export class Err {
  public constructor(
    public name: string,
    public positionStart: Position,
    public positionEnd: Position,
    public description: string,
  ) {}

  public formatted(): string {
    return ` File Name: ${this.positionStart.fileName}\n Line: ${this
      .positionStart.line +
      1} \n Name: ${this.name}\n Description: ${this.description}`;
  }
}
