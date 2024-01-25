import { defineStore } from "pinia";

const LIGHTMODE = 'lightmode';
export const LOBBY_REFRESH_INTERVAL = 5000; //ms
export const GAME_REFRESH_INTERVAL = 10000; //ms
export const usePreferencesStore = defineStore({
    id: 'pref',
    // state: () => ({
    // }),
    getters: {
        getLightmodePreference: () => {
            return localStorage.getItem(LIGHTMODE) === 'true';
        }
    },
    actions: {
        setLightmodePreference(lightMode) {
            localStorage.setItem(LIGHTMODE, lightMode);
        }
    },
});
