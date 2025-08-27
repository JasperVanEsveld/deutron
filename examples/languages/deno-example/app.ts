import { Deutron } from "./deutron.ts";

const deutron = new Deutron({
    title: "Deno Example",
});

deutron.onMessage((event) => {
    deutron.message(
        event.from,
        `Copy that ${event.from}, I received "${event.data}"`
    );
});
