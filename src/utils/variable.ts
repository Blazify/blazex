import { TYPES } from "./constants.ts";

export class Variable<Type> {
    constructor(public value: Type, public type: TYPES, public reassignable: boolean) {}
}