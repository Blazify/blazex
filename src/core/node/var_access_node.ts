import { Position } from "../../error/position.ts";
import { Token } from "../token.ts";

export class VarAcessNode {
    public positionStart: Position;
    public positionEnd: Position;
    constructor(public token: Token) {
        this.positionStart = token.positionStart!;
        this.positionEnd = token.positionEnd!;
    }
}