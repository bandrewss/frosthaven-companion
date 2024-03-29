
function gamestateTransition()
{
    let url = "gamestate/transition";
    let options =
    {
        method: "POST"
    };

    fetch(url, options);
}

function playerTransition()
{
    let url = "gamestate/next-player";
    let options =
    {
        method: "POST"
    };

    fetch(url, options);
}
