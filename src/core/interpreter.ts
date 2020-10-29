import { Err } from "../error/err.ts";
import { RuntimeError } from "../error/runtimeerr.ts";
import { DIVIDE, MINUS, MULTIPLY, PLUS, POWER } from "../utils/constants.ts";
import { Context } from "./context.ts";
import { BinOpNode } from "./node/binary_op_node.ts";
import { NumberNode } from "./node/number_nodes.ts";
import { UnaryOpNode } from "./node/unary_op_node.ts";
import { VarAcessNode } from "./node/var_access_node.ts";
import { VarAssignNode } from "./node/var_assign_node.ts";
import { Number as MyNumber } from "./number.ts";
import { RuntimeResult } from "./runtime_result.ts";

export class Interpreter {
  public visit(
    node: BinOpNode | NumberNode | UnaryOpNode | VarAssignNode | VarAcessNode,
    context: Context,
  ): RuntimeResult {
    if (node instanceof BinOpNode) {
      return this.visitBinOpNode(node, context);
    } else if (node instanceof NumberNode) {
      return this.visitNumberNode(node, context);
    } else if (node instanceof UnaryOpNode) {
      return this.visitUnaryOpNode(node, context);
    } else if(node instanceof VarAcessNode) { 
      return this.visitVarAccessNode(node, context);
    } else if(node instanceof VarAssignNode) {
      return this.visitVarAssignNode(node, context);
    } else {
      return this.noVisitMethod(node, context) as unknown as RuntimeResult;
    }
  }

  public noVisitMethod(
    _node: UnaryOpNode | BinOpNode | NumberNode | VarAssignNode | VarAcessNode,
    _context: Context,
  ) {
    throw "No visit method found for node type\n";
  }

  public visitVarAccessNode(node: VarAcessNode, context: Context): RuntimeResult {
    const res = new RuntimeResult();
    const varName = node.token.value;
    const varValue = context.symbolTable?.get(varName as string);
    if(!varValue) {
      return res.failure(new RuntimeError(node.positionStart, node.positionEnd, `${varName} is not defined yet!`, context));
    }
    return res.success(varValue);
  }

  public visitVarAssignNode(node: VarAssignNode, context: Context): RuntimeResult {
    const res = new RuntimeResult();
    const varName = node.name.value;
    const varValue = res.register(this.visit(node.value, context));
    if(!res.error) {
      return res;
    }
    context.symbolTable?.set(varName as string, varValue!)
    return res.success(varValue!);
  }

  public visitBinOpNode(node: BinOpNode, context: Context) {
    const res = new RuntimeResult();
    const left: MyNumber = res.register(
      this.visit(node.leftNode, context) as unknown as MyNumber,
    )!;
    if (res.error) return res;
    const right: MyNumber = res.register(
      this.visit(node.rightNode, context) as unknown as MyNumber,
    )!;
    if (res.error) return res;
    let final!: MyNumber;
    let err!: Err;

    if (node.opToken.type === PLUS) {
      const { result, error } = left.addTo(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === MINUS) {
      const { result, error } = left.subBy(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === MULTIPLY) {
      const { result, error } = left.multiBy(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === DIVIDE) {
      const { result, error } = left.divBy(right)!;
      if (error) err = error;
      else if (result) final = result;
    } else if (node.opToken.type === POWER) {
      const { result, error } = left.powBy(right)!;
      if (error) err = error;
      else if (result) final = result;
    }

    if (err) return res.failure(err);

    return res.success(final.setPosition(node.positionStart, node.positionEnd));
  }

  public visitNumberNode(node: NumberNode, context: Context) {
    const res = new RuntimeResult();
    return res.success(
      new MyNumber(Number(node.token.value!)).setPosition(
        node.positionStart,
        node.positionEnd,
      ).setContext(context),
    );
  }

  public visitUnaryOpNode(node: UnaryOpNode, context: Context) {
    const res = new RuntimeResult();
    let number = res.register(
      this.visit(node.node, context) as unknown as MyNumber,
    )!;
    let err!: Err;
    if (res.error) return res;

    if (node.opToken.type === MINUS) {
      const { result, error } = number.multiBy(new MyNumber(-1))!;
      if (error) err = error;
      else if (result) number = result;
    }

    if (err) return res.failure(err);

    return res.success(
      number.setPosition(node.positionStart, node.positionEnd),
    );
  }
}
