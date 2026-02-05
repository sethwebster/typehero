interface ASTNode {
  accept<T>(visitor: Visitor<T>): T;
}

class Literal implements ASTNode {
  constructor(public value: number) {}
  accept<T>(visitor: Visitor<T>): T {
    return visitor.visitLiteral(this);
  }
}

class BinaryOp implements ASTNode {
  constructor(public left: ASTNode, public op: string, public right: ASTNode) {}
  accept<T>(visitor: Visitor<T>): T {
    return visitor.visitBinaryOp(this);
  }
}

class UnaryOp implements ASTNode {
  constructor(public op: string, public operand: ASTNode) {}
  accept<T>(visitor: Visitor<T>): T {
    return visitor.visitUnaryOp(this);
  }
}

interface Visitor<T> {
  visitLiteral(node: Literal): T;
  visitBinaryOp(node: BinaryOp): T;
  visitUnaryOp(node: UnaryOp): T;
}

class Evaluator implements Visitor<number> {
  visitLiteral(node: Literal): number {
    return node.value;
  }

  visitBinaryOp(node: BinaryOp): number {
    const left = node.left.accept(this);
    const right = node.right.accept(this);

    switch (node.op) {
      case '+': return left + right;
      case '-': return left - right;
      case '*': return left * right;
      case '/': return left / right;
      default: throw new Error(`Unknown op: ${node.op}`);
    }
  }

  visitUnaryOp(node: UnaryOp): number {
    const operand = node.operand.accept(this);
    return node.op === '-' ? -operand : operand;
  }
}

// Usage:
// const ast = new BinaryOp(new Literal(5), '+', new Literal(3));
// const result = ast.accept(new Evaluator()); // 8
