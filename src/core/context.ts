import { Position } from "../error/position.ts";

export class Context {
  constructor(
    public displayName: string,
    public parent?: Context,
    public parentEntryPosition?: Position,
  ) {}
}
