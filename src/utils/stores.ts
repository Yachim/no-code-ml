import { writable } from "svelte/store";

export const saveFunc = writable<() => void>(() => {
	console.log("saveFunc not implemented")
})

export const isNotSaved = writable(false);

export const currentNetId = writable("");
