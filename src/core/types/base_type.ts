import { Err } from "../../error/err.ts";
import { Position } from "../../error/position.ts";
import { RuntimeError } from "../../error/runtimeerr.ts";
import { TYPES } from "../../utils/constants.ts";
import { Context } from "../context.ts";
import { RuntimeResult } from "../runtime_result.ts";
import { Number } from "./number.ts";

export class BaseType {
  public positionStart: Position | null = null;
  public positionEnd: Position | null = null;
  public context: Context | null = null;

  constructor() {
    this.setPosition();
    this.setContext();
  }

  public represent(): any {
    throw new Error("No represent method");
  }

  public get value(): any {
    return new RuntimeResult().failure(
      new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "No value specified",
        this.context!,
      ),
    );
  }

  public execute(_args: any[]): any {
    return new RuntimeResult().failure(
      new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "No execute method specified",
        this.context!,
      ),
    );
  }

  public get type(): TYPES {
    return "UNKNOWN" as TYPES;
  }

  public setPosition(
    start: Position | null = null,
    end: Position | null = null,
  ) {
    this.positionStart = start;
    this.positionEnd = end;
    return this;
  }

  public addTo(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public subBy(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public multiBy(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public divBy(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public powBy(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public equals(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public notEquals(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public lessThan(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public greaterThan(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public lessThanEquals(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public greaterThanEquals(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public and(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public or(
    _other: unknown,
  ): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public not(): { result: Number | null; error: Err | null } {
    return {
      result: null,
      error: new RuntimeError(
        this.positionStart!,
        this.positionEnd!,
        "",
        this.context!,
      ),
    };
  }

  public setContext(context: Context | null = null) {
    this.context = context;
    return this;
  }

  public clone(): any {
    throw new Error("No Clone method defined");
  }
}
