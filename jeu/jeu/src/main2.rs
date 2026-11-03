fn afficher(table: &Vec<char>){
        println!("{}",table);
}


fn f(c: char)->i32{
    if c=='.'{
        0
    }else{
        1
    }
}


fn nouveau_plateau(plateau: &Vec<char>)->Vec<char>{
    let n=plateau.len().isqrt();
    let mut plan:Vec<char> =  vec!['.'; (n+2)*(n+2)];
    for i in 1..=n{
        for j in 1..=n{
            plan[(i+1)*(n+2) + (j+1))]=plateau[n*i+j];
        }
    };
    let mut nv_plateau= (*plateau).clone();
    for i in 0..n{
        for j in 0..n{
            let mut a:i32 =-f(plateau[i*n+j]);
            for k in (i+1)..=(i+3){
                for l in (j+1)..=(j+3){
                    a+= f(plan[k*(n+2)+l]);
                }
            }
            if plateau[i*n+j]=='.' && a==3{
                nv_plateau[i*n+j]='#'
            } else if !((a==2 || a==3)) && plateau[i*n+j]=='#'{
                nv_plateau[i*n+j]='.'
            }
        }
    };
    nv_plateau
}





fn main() {

//-----------------------SAISIE DU PLATEAU----------------------------------------------------------
    //let mut plateau = Vec::new();
    //for _ in 0..5 {
    //    plateau.push(vec!['.', '#', '.', '#', '.']);
    //}
    let plateau= vec!['.','.','.','.','.','.','.','.','.','.','#','#','#','#','#','.','.','.','.','.','.','.','.','.','.'];
    afficher(&plateau);    

//-----------------------JOUER EN ITERANT nb FOIS-------------------------------------
    let nv_plateau= nouveau_plateau(&plateau);
    println!("\nnouveau plateau : ");
    afficher(&nv_plateau);
}