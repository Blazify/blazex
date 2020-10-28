import { Err } from "../error/err.ts";
import { DIVIDE, MINUS, MULTIPLY, PLUS } from "../utils/constants.ts";
import { BinOpNode } from "./node/binary_op_node.ts";
import { NumberNode } from "./node/number_nodes.ts";
import { UnaryOpNode } from "./node/unary_op_node.ts";
import { Number as MyNumber } from "./number.ts";
import { RuntimeResult } from "./runtime_result.ts";

export class Interpreter {
    public visit(node: BinOpNode | NumberNode | UnaryOpNode): RuntimeResult {
        let visitNode: string;
        if(node instanceof BinOpNode) {
            visitNode = "visitBinOpNode";
        } else if(node instanceof NumberNode) {
            visitNode = "visitNumberNode";
        } else {
            visitNode = "visitUnaryOpNode"
        }

        const method = this[visitNode as keyof Interpreter] ?? this.noVisitMethod
        return this[method.name as keyof Interpreter](node as any) as RuntimeResult;
    }

    public noVisitMethod(_node: UnaryOpNode | BinOpNode | NumberNode) {
        throw "No visit method found"
    }

    public visitBinOpNode(node: BinOpNode) {
        const res = new RuntimeResult();
        const left: MyNumber = res.register(this.visit(node.leftNode) as unknown as MyNumber)!;
        if(res.error) return res
        const right: MyNumber = res.register(this.visit(node.rightNode) as unknown as MyNumber)!;
        if(res.error) return res
        let final!: MyNumber;
        let err!: Err;

        if(node.opToken.type === PLUS) {
            const { result, error } = left.addTo(right)!;
            if(error) err = error
            else if(result) final = result;
        } else if(node.opToken.type === MINUS) {
            const { result, error } = left.subBy(right)!;
            if(error) err = error
            else if(result) final = result;        
        } else if(node.opToken.type === MULTIPLY) {
            const { result, error } = left.multiBy(right)!;
            if(error) err = error
            else if(result) final = result;
        } else if(node.opToken.type === DIVIDE) {
            const { result, error } = left.divBy(right)!;
            if(error) err = error
            else if(result) final = result;
        }

        if(err) return res.failure(err)

        return res.success(final.setPosition(node.positionStart, node.positionEnd));
    }

    public visitNumberNode(node: NumberNode) {
        return new MyNumber(Number(node.token.value!)).setPosition(node.positionStart, node.positionEnd);
    }

    public visitUnaryOpNode(node: UnaryOpNode) {
        const res = new RuntimeResult();
        let number = res.register(this.visit(node.node) as unknown as MyNumber)!;
        let err!: Err;
        if(res.error) return res

        if(node.opToken.type === MINUS) {
            const { result, error } = number.multiBy(new MyNumber(-1))!
            if(error) err = error
            else if(result) number = result;
        }

        if(err) return res.failure(err);

        return res.success(number.setPosition(node.positionStart, node.positionEnd));
    }
}