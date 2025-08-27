import { Deutron } from "./deutron.ts";

const deutron = new Deutron({
    no_decorations: true,
    title: "Custom Decoration Example",
});

deutron.onMessage((event) => {
    deutron.message(
        event.from,
        `Copy that ${event.from}, I received "${event.data}"`
    );
});
