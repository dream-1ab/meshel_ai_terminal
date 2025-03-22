/**
 * @author مۇختەرجان مەخمۇت
 * @email ug-project@outlook.com
 * @create date 2025-03-22 03:53:35
 * @modify date 2025-03-22 03:53:35
 * @desc [description]
 */
import {sleep} from "core/threading"
// import {sendMessage} from "core/messaging"

let index = 0
while (true) {
    // let bytes = new Uint8Array(1024)
    console.log("hello world!", index)
    index+= 1
    sleep(16)
}

let count = 0
export function uiMain(context) {
    return Column({
        children: [
            Text(`Count: ${count++}`),
            ElevatedButton({
                child: Text("hello world"),
                onTap: () => {
                    sendMessage({
                        action: "increaseCount",
                        value: 2
                    })
                }
            }),
            Row({
                children: [

                ]
            })
        ]
    })
}
