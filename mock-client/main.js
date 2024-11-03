
const BTN_ID = 'btn-send';

let iteration = 0;

function createWebSocket() {
    const ws = new WebSocket("ws://localhost:8080");

    ws.addEventListener("open", () => {

        console.log("WebSocket opened");

        {
            const msg = `{ "message": "opened" }`;
            ws.send(msg);
        }

        // bind button
        const btn = document.getElementById(BTN_ID);

        if (!(btn instanceof HTMLElement)) {
            console.error("Could not get button element.");
            return;
        }

        btn.onclick = () => {
            console.log("Button clicked. Sending message");
            const msg = `{ "message": "test", "iteration": ${iteration++} }`;
            ws.send(msg);
        };
    });
}

createWebSocket();
