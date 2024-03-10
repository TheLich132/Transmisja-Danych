#![allow(non_snake_case)]

fn generate_parity_matrix(m: usize) -> Vec<Vec<usize>> {
    let size: usize = usize::pow(2, m as u32);
    let mut parity_matrix: Vec<Vec<usize>> = vec![];
    for i in 1..size {
        if !usize::is_power_of_two(i){
            let bits: String = format!("{:04b}", i as u32);
            let mut temp: Vec<usize> = bits.chars().map(|c| String::from(c).parse::<usize>().unwrap()).collect();
            temp.reverse();
            parity_matrix.push(temp);
        }
    }
    parity_matrix
}

fn create_identity_matrix(size: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; size]; size];

    for i in 0..size {
        matrix[i][i] = 1;
    }

    matrix
}

fn transpose(matrix: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    if matrix.is_empty() {
        return Vec::new();
    }
    
    let num_rows: usize = matrix.len();
    let num_cols: usize = matrix[0].len();
    
    let mut transposed: Vec<Vec<usize>> = vec![vec![0; num_rows]; num_cols];
    
    for i in 0..num_rows {
        for j in 0..num_cols {
            transposed[j][i] = matrix[i][j];
        }
    }
    
    transposed
}

fn matrix_multiply(vector: Vec<usize>, matrix: Vec<Vec<usize>>) -> Vec<usize> {
    let rows: usize = matrix.len();
    let cols: usize = matrix[0].len();

    if vector.len() != rows {
        panic!("Liczba kolumn wektora musi być równa liczbie wierszy macierzy");
    }

    println!("DEBUG: Matrix_multiply");
    for v in matrix.iter(){
        println!("{:?}", v);
    }

    println!("{:?}\n", vector);

    let mut result: Vec<usize> = vec![0; cols];

    for i in 0..cols {
        for j in 0..rows {
            print!("({}, {}), ", vector[j], matrix[j][i]);
            result[i] += vector[j] * matrix[j][i];
        }
        println!()
    }

    println!("{:?}\n\n", result);
    result
}

fn column_merge(matrix1: Vec<Vec<usize>>, matrix2: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let rows1: usize = matrix1.len();
    let rows2: usize = matrix2.len();

    if rows1 != rows2 {
        panic!("Podane macierze mają różną liczbę wierszy");
    }

    let cols1: usize = matrix1[0].len();
    let cols2: usize = matrix2[0].len();
    let merged_cols: usize = cols1 + cols2;

    let mut merged_matrix = Vec::with_capacity(rows1);

    for row in 0..rows1 {
        let mut merged_row = Vec::with_capacity(merged_cols);

        for col in 0..cols1 {
            merged_row.push(matrix1[row][col]);
        }

        for col in 0..cols2 {
            merged_row.push(matrix2[row][col]);
        }

        merged_matrix.push(merged_row);
    }

    merged_matrix
}

pub fn ham_74_coding(msg: &[usize]) -> Vec<usize>{
    let mut code: Vec<usize> = vec!(0,0,0,0,0,0,0);
    code[2] = msg[0];
    code[4] = msg[1];
    code[5] = msg[2];
    code[6] = msg[3];
    code[0] = code[2] ^ code[4] ^ code[6];
    code[1] = code[2] ^ code[5] ^ code[6];
    code[3] = code[4] ^ code[5] ^ code[6];

    code
}

pub fn ham_74_decoding(msg_ham74: &[usize]) -> Vec<usize>{
    let mut code_copy: Vec<usize> = msg_ham74.to_owned().to_vec();
    let mut msg: Vec<usize> = vec!(0,0,0,0);

    let x0_prim: usize = msg_ham74[2] ^ msg_ham74[4] ^ msg_ham74[6];
    let x1_prim: usize = msg_ham74[2] ^ msg_ham74[5] ^ msg_ham74[6];
    let x3_prim: usize = msg_ham74[4] ^ msg_ham74[5] ^ msg_ham74[6];

    let x0: usize = msg_ham74[0] ^ x0_prim;
    let x1: usize = msg_ham74[1] ^ x1_prim;
    let x3: usize = msg_ham74[3] ^ x3_prim;

    let sum: usize = x0 * usize::pow(2, 0) + x1 * usize::pow(2, 1) + x3 * usize::pow(2,2);

    if sum != 0 {
        code_copy[sum-1] = (code_copy[sum-1] == 0) as usize;
    }

    msg[0] = code_copy[2];
    msg[1] = code_copy[4];
    msg[2] = code_copy[5];
    msg[3] = code_copy[6];

    msg
}

pub fn ham_1511_coding(msg: &[usize]) -> Vec<usize>{
    let m: usize = 15 - msg.len();
    let P: Vec<Vec<usize>> = generate_parity_matrix(m);

    let I: Vec<Vec<usize>> = create_identity_matrix(msg.len());
    let G: Vec<Vec<usize>> = column_merge(P, I);

    let c: Vec<usize> = matrix_multiply(msg.to_vec(), G);
    let c_end: Vec<usize> = c.iter().copied().map(|x| if x % 2 == 0 {0} else {1}).collect();
    c_end
}

pub fn ham_1511_decoding(msg_ham1511: &[usize]) -> Vec<usize>{
    let m: usize = msg_ham1511.len() - 11; 
    let mut code_copy: Vec<usize> = msg_ham1511.to_owned().to_vec();

    let P: Vec<Vec<usize>> = generate_parity_matrix(m);
                        
    let I: Vec<Vec<usize>> = create_identity_matrix(m);
    let H: Vec<Vec<usize>> = column_merge(I, transpose(P));

    let s: Vec<usize> = matrix_multiply(msg_ham1511.to_vec(), transpose(H)).iter().copied().map(|x| if x % 2 == 0 {0} else {1}).collect();
    let sum: usize = s.iter().enumerate().map(|x| x.1 * usize::pow(2, x.0 as u32)).sum();
    println!("{}", sum);
    if sum != 0 {
        code_copy[sum-1] = (code_copy[sum-1] == 0) as usize;
    }
    
    code_copy.iter().skip(m).copied().collect()
}
