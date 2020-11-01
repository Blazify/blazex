import { RuntimeError } from "../../error/runtimeerr.ts";
import { Nodes, TYPES } from "../../utils/constants.ts";
import { SymbolTable } from "../../utils/symbol_table.ts";
import { Variable } from "../../utils/variable.ts";
import { Context } from "../context.ts";
import { Interpreter } from "../interpreter.ts";
import { RuntimeResult } from "../runtime_result.ts";
import { Token } from "../token.ts";
import { BaseType } from "./base_type.ts";

export class Function extends BaseType {
  constructor(
    public name: string,
    public bodyNode: Nodes,
    public argNames?: Token[],
  ) {
    super();
    if (!name) this.name = "anonymous";
  }

  public execute(args: BaseType[]) {
    const res = new RuntimeResult();
    const interpreter = new Interpreter();

    const context = new Context(
      this.name,
      this.context ?? undefined,
      this.positionStart ?? undefined,
    );
    context.symbolTable = new SymbolTable();
    context.parent = this.context!;

    if (args.length > this.argNames!.length) {
      return res.failure(
        new RuntimeError(
          this.positionStart!,
          this.positionEnd!,
          `Too many args passed in ${this.name} function`,
          this.context!,
        ),
      );
    }

    if (args.length < this.argNames!.length) {
      return res.failure(
        new RuntimeError(
          this.positionStart!,
          this.positionEnd!,
          `Too less args passed in ${this.name} function`,
          this.context!,
        ),
      );
    }

    for (let i = 0; i < args.length; i++) {
      const name = this.argNames![i];
      const value = args[i];
      if (name.type !== value.type) {
        return res.failure(
          new RuntimeError(
            this.positionStart!,
            this.positionEnd!,
            `${name.type} doesn't satisfy ${value.type}`,
            context,
          ),
        );
      }
      value.setContext(context);
      context.symbolTable.set(
        name.value as string,
        new Variable(value, value.type, true),
      );
    }

    const value = res.register(interpreter.visit(this.bodyNode, context));
    if (res.error) return res;
    return res.success(value!);
  }

  public clone() {
    return new Function(this.name, this.bodyNode, this.argNames).setPosition(
      this.positionStart,
      this.positionEnd,
    ).setContext(this.context);
  }

  public represent(): string {
    return `<function ${this.name}>`;
  }

  get type(): TYPES {
    return `Function: ${this.bodyNode.type}` as TYPES
  }
}
