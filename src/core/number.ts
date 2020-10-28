import { Position } from "../error/position.ts";
import { Lexer } from "./lexer.ts";

export class Number {
    public positionStart!: Position | null;
    public positionEnd!: Position | null;
    constructor(public value: number) {
        this.setPosition();
    }

    public setPosition(start: Position | null = null, end: Position | null = null) {
        this.positionStart = start;
        this.positionEnd = end;
        return this;
    }

    public addTo(other: Number) {
        if(other instanceof Number) {
            return new Number(this.value + other.value);
        }
    }

    public subBy(other: Number) {
        if(other instanceof Number) {
            return new Number(this.value - other.value);
        }
    }

    public multiBy(other: Number) {
        if(other instanceof Number) {
            return new Number(this.value * other.value);
        }
    }

    public divBy(other: Number) {
        if(other instanceof Number) {
            return new Number(this.value / other.value);
        }
    }

    public represent() {
        const { tokens, errors } = new Lexer("Number", String(this.value)).makeTokens();
        if(errors) throw errors.forEach(e => console.log(e.formatted()));
        return `${tokens![0].type}: ${tokens![0].value}`
    }
}
