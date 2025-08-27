import { Deutron } from "./deutron.ts";

const deutron = new Deutron({
    title: "Typescript Example",
    url: "./frontend/index.html",
});

deutron.onMessage((event) => {
    deutron.message(
        event.from,
        `Copy that ${event.from}, I received "${event.data}"`
    );
});
