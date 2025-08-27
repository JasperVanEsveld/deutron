from deutron import Deutron

deutron = Deutron({
    "title": "Python Example",
})


def message(event):
    response = "Processed message: " + event["data"]
    deutron.message(event["from"], response)


deutron.on_message(message)
deutron.start()
