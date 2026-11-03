fn afficher(table: &Vec<Vec<char>>){
    for ligne in table {
        for cellule in ligne {
            print!("{}", cellule);
        }
        println!();
    }
}


fn f(c: char)->i32{
    if c=='.'{
        0
    }else{
        1
    }
}


fn nouveau_plateau(plateau: &Vec<Vec<char>>)->Vec<Vec<char>>{
    let n=plateau.len();
    let mut plan:Vec<Vec<char>> =  vec![ vec!['.'; n+2]; n+2];
    for i in 0..n{
        for j in 0..n{
            plan[i+1][j+1]=plateau[i][j];
        }
    }
    let mut nv_plateau= (*plateau).clone();
    for i in 0..n{
        for j in 0..n{
            let mut a:i32 =-f(plateau[i][j]);
            for k in i..=(i+2){
                for l in j..=(j+2){
                    a+= f(plan[k][l]);
                }
            }
            if plateau[i][j]=='.' && a==3{
                nv_plateau[i][j]='#'
            } else if !((a==2 || a==3)) && plateau[i][j]=='#'{
                nv_plateau[i][j]='.'
            }
        }
    }
    nv_plateau
}


fn mort_marge(plateau: &Vec<Vec<char>>)-> bool{
    let mut test=true;
    for ligne in plateau{
        for cellule in ligne{
            test= test && *cellule=='.';
            if test{
                return test;
            }
        }
    }
    return test;
}


fn iterer(nb:i32, plateau: &Vec<Vec<char>>){
    let plateau_actuel=plateau.clone();
    for i in 0..nb{
        //si les vivants atteigent la marge on élargit notre plateau et on rejoue
        let nv_plateau= {
            if mort_marge( &plateau_actuel ){ 
                nouveau_plateau( &plateau_actuel )
            }else{
                let n=plateau.len();
                let mut plateau_elargi:Vec<Vec<char>> =  vec![ vec!['.'; n+2]; n+2];
                for i in 0..n{
                    for j in 0..n{
                        plateau_elargi[i+1][j+1]=plateau[i][j];
                    }
                }
                nouveau_plateau( &plateau_elargi )
            }
        };
        
        let plateau_actuel= nv_plateau;
        println!("\nITERATION N°= {}",i);
        afficher(&plateau_actuel);
    }
}




fn main() {
//----------------------SAISIE DU NOMBRE D'ITERATIONS-----------------------------------------------------
    use std::io;
    println!("donner le nombre d'itérations :");
    let mut ch = String::new();
    io::stdin()
        .read_line(&mut ch)//ou: input_line
        .expect("Échec de la lecture de la ligne");//Failed to read line
    let nb: i32 = ch.trim().parse().expect("Veuillez entrer un nombre");

//-----------------------SAISIE DU PLATEAU----------------------------------------------------------
    //let mut plateau = Vec::new();
    //for _ in 0..5 {
    //    plateau.push(vec!['.', '#', '.', '#', '.']);
    //}
    let plateau= vec![vec!['.','.','.','.','.'],vec!['.','.','.','.','.'],vec!['#','#','#','#','#'],vec!['.','.','.','.','.'],vec!['.','.','.','.','.']];
    afficher(&plateau);    

//-----------------------JOUER EN ITERANT nb FOIS-------------------------------------
    iterer(nb,&plateau);
}
