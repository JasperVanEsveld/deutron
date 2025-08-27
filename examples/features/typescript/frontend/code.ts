/// <reference path="./deutron.d.ts" />

function sendMessage() {
    const message = (document.getElementById("message") as HTMLInputElement)
        .value;
    deutron.messageBackend(message);
}

deutron.onMessage((event) => {
    document.getElementById("parent")!.textContent = event.data;
});

deutron.onInfo((event) => {
    console.log(event);
});
