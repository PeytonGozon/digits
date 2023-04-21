use std::vec;

fn main() {
    let target: i32 = 463;
    let numbers = vec![5, 7, 9, 11, 13, 23];

    let start = GameState::new(numbers, target);
    let solutions = solve(start);

    if let Some(solution) = find_shortest_solution(solutions) {
        println!("{:#?}", solution);
    } else {
        println!("No solutions found!");
    }
}

fn solve(game_state: GameState) -> Vec<GameState> {
    let mut solutions = vec![];
    let mut work_queue = vec![game_state];

    while let Some(game_state) = work_queue.pop() {
        for child in game_state.apply_operations() {
            if child.is_solution() {
                solutions.push(child);
            } else {
                work_queue.push(child);
            }
        }
    }

    solutions
}

fn find_shortest_solution(solutions: Vec<GameState>) -> Option<GameState> {
    // Find the solution with the fewest moves.
    solutions.into_iter().reduce(|acc, e| {
        if acc.history.len() < e.history.len() {
            acc
        } else {
            e
        }
    })
}

#[derive(Clone, Copy)]
struct Operation {
    operands: (Operand, Operand),
    operation: OperationKind,
    value: Option<Operand>
}

impl Operation {
    fn new(operands: (Operand, Operand), operation: OperationKind) -> Self {
        Self {
            operands,
            operation,
            value: Self::apply(operands, operation)
        }
    }

    fn apply(pair: (Operand, Operand), op: OperationKind) -> Option<Operand> {
        match op {
            OperationKind::Add => Some(pair.0 + pair.1),
            OperationKind::Subtract => {
                // Only return the value if it's non-negative
                if pair.0 >= pair.1 {
                    Some(pair.0 - pair.1)
                } else {
                    None
                }
            },
            OperationKind::Multiply => Some(pair.0 * pair.1),
            OperationKind::Divide => {
                // Only return the value if they cleanly divide.
                if pair.1 != 0 && pair.0 % pair.1 == 0 {
                    Some(pair.0 / pair.1)
                } else {
                    None
                }
            }
        }
    }
}

impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (op1, op2) = self.operands;

        let value = match self.value {
            Some(value) => value.to_string(),
            None => "ø".to_string() // Invalid operation.
        };
        
        write!(f, "{} {:?} {} = {}", op1, self.operation, op2, value)
    }
}

#[derive(Debug)]
struct GameState {
    numbers: Operands,
    target: Operand,
    history: Vec<Operation>
}

impl GameState {
    fn new(numbers: Operands, target: Operand) -> Self {
        Self {
            numbers,
            target,
            history: vec![]
        }
    }

    fn apply_operations(&self) -> Vec<GameState> {

        let mut child_gamestates = vec![];

        // Compute all pairs of numbers.
        let pairs = get_all_pairs_ordered(&self.numbers);

        for operation_kind in [OperationKind::Add, OperationKind::Subtract, OperationKind::Multiply, OperationKind::Divide] {
            for pair in &pairs {
                let operation = Operation::new(pair.clone(), operation_kind);
                
                // If the operation succeeded
                if let Some(value) = operation.value {

                    let mut history = self.history.clone();
                    history.push(operation);

                    // Remove the two operands, apply the operation, and push the new value into the history.
                    let mut numbers: Operands = self.numbers.clone().into_iter().filter(|&n| n != pair.0 && n != pair.1).collect();
                    numbers.push(value);

                    child_gamestates.push(
                        Self {
                            history,
                            numbers,
                            target: self.target
                        }
                    )
                }

            }
        }

        child_gamestates
    }

    fn is_solution(&self) -> bool {
        self.numbers.contains(&self.target)
    }
}

type Operand = i32;
type Operands = Vec<Operand>;


#[derive(Clone, Copy)]
enum OperationKind {
    Add, 
    Subtract,
    Multiply,
    Divide
}

impl std::fmt::Debug for OperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Add => '+',
            Self::Subtract => '-',
            Self::Multiply => '*',
            Self::Divide => '/'
        })
    }
}

// TODO: redo in a functional style!
fn get_all_pairs_ordered(numbers: &Operands) -> Vec<(Operand, Operand)> {
    let n = numbers.len();

    let mut pairs = Vec::new();

    for i in 0..n {
        for j in (i+1)..n {

            let &v0 = numbers.get(i).expect("index i: 1 <= i < n");
            let &v1 = numbers.get(j).expect("index j: i+1 <= j < n");

            pairs.push((v0, v1));
            pairs.push((v1, v0));
        }
    }

    pairs
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn all_pairs_empty() {
        // Expect no pairs
        let numbers = vec![];
        let expected_pairs = vec![];
        test_all_pairs(&numbers, expected_pairs);
    }

    #[test]
    fn all_pairs_single() {
        let numbers = vec![1];
        let expected_pairs = vec![];
        test_all_pairs(&numbers, expected_pairs);
    }

    #[test]
    fn all_pairs_two() {
        let numbers = vec![1, 2];
        let expected_pairs = vec![(1, 2), (2, 1)];
        test_all_pairs(&numbers, expected_pairs);
    }

    #[test]
    fn all_pairs_three() {
        let numbers = vec![1, 2, 3];
        let expected_pairs = vec![
            (1, 2), (2, 1),
            (1, 3), (3, 1),
            (2, 3), (3, 2)
        ];
        test_all_pairs(&numbers, expected_pairs);
    }

    fn test_all_pairs(numbers: &Vec<i32>, expected_pairs: Vec<(i32, i32)>) {
        let actual_pairs = get_all_pairs_ordered(numbers);
        
        assert!(actual_pairs.len() == expected_pairs.len(), "Incorrect number of pairs created. Created: {:?}, expected: {:?}", actual_pairs, expected_pairs);

        // Check that all pairs are equal
        assert!(actual_pairs
            .iter()
            .all(|pair| expected_pairs.contains(pair)), 
            "All actual pairs are expected");

        assert!(actual_pairs
            .iter()
            .all(|pair| expected_pairs.contains(pair)), "All expected pairs are actually created.");
    }
}