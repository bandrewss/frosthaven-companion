
async function putPlayer()
{
    let initiative_row = document.getElementById("new_player");
    let row_array = Array.from(initiative_row.cells);

    let submit =
    {
        name: row_array[3].firstChild.value,
        initiative: Number(row_array[1].firstChild.value),
        is_monster: row_array[2].firstChild.checked
    };

    let url = "player";
    let options =
    {
        method: "PUT",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(submit)
    };

    await fetch(url, options);

    location.reload();
}


async function postPlayerInitiative(row_id)
{
    let initiative_row = document.getElementById(row_id);
    let row_array = Array.from(initiative_row.cells);

    let submit =
    {
        name: row_array[3].innerHTML,
        initiative: Number(row_array[1].firstChild.value),
    };

    let url = "player/initiative";
    let options =
    {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(submit)
    };

    await fetch(url, options);

    location.reload();
}

async function deletePlayerInitiative(player_name)
{
    let submit =
    {
        name: player_name
    };

    let url = "player/initiative";
    let options =
    {
        method: "DELETE",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(submit)
    };

    await fetch(url, options);

    location.reload();
}

async function deletePlayer(player_name)
{
    let submit =
    {
        name: player_name
    };

    let url = "player";
    let options =
    {
        method: "DELETE",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(submit)
    };

    await fetch(url, options);

    location.reload();
}
