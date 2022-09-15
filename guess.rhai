
fn guess(input) {
    if input == target{
        print(`Congrats! You Won!!!`);
        return true;
    }
    else if input > target{
        print(`INPUT TOO BIG`);
        return false;
    }
    else{
        print(`INPUT TOO SMALL`);
        return false;
    }
}
