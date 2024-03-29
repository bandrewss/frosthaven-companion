
function reload()
{
    location.reload();
}

function pollReload(loadedGamestate)
{
    setTimeout(pollForUpdate, 499, loadedGamestate, 500, reload);
}

function pollFunction(loadedGamestate, call_on_update)
{
    setTimeout(pollForUpdate, 499, loadedGamestate, 500, call_on_update);
}

async function pollForUpdate(loadedGamestate, interval_milliseconds, call_on_update)
{
    let update_url = "gamestate/current";

    let currentGamestateRequest = await fetch(update_url);
    let currentGamestate = await currentGamestateRequest.text();

    console.log(currentGamestate);

    if (currentGamestate !== loadedGamestate)
    {
        call_on_update();
    }

    await setTimeout(pollForUpdate, interval_milliseconds, loadedGamestate, interval_milliseconds, call_on_update);
}
