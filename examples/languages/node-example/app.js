import { Deutron } from "./deutron.js";

const deutron = new Deutron({
    title: "Node Example",
});

deutron.onMessage((event) => {
    deutron.message(
        event.from,
        `Copy that ${event.from}, I received "${event.data}"`
    );
});
