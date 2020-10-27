export class Position {
  public constructor(
    public index: number,
    public line: number,
    public column: number,
    public fileName: string,
    public fileContent: string,
  ) {}

  public advance(char: string): Position {
    this.index += 1;
    this.column += 1;
    if (char.includes("\n")) {
      this.line += 1;
      this.column = 0;
    }

    return this;
  }

  public clone(): Position {
    return new Position(
      this.index,
      this.line,
      this.column,
      this.fileName,
      this.fileContent,
    );
  }
}
