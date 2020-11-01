import { Err } from "../error/err.ts";
import { BaseType } from "./types/base_type.ts";
import { Number } from "./types/number.ts";

export class RuntimeResult {
  public value: BaseType | null = null;
  public error: Err | null = null;

  public register(res: RuntimeResult | Number) {
    if (res instanceof RuntimeResult) {
      if (res.error) this.error = res.error;
      return res.value;
    }
    this.value = res;
    return this.value;
  }

  public success(value: BaseType) {
    this.value = value as unknown as Number;
    return this;
  }

  public failure(err: Err) {
    this.error = err;
    return this;
  }
}
