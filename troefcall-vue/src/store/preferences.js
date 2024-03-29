import { defineStore } from "pinia";

const LIGHTMODE = 'lightmode';
export const LOBBY_REFRESH_INTERVAL = 5000; //ms
export const GAME_REFRESH_INTERVAL = 2000; //ms
export const usePreferencesStore = defineStore({
    id: 'pref',
    // state: () => ({
    // }),
    getters: {
        getLightmodePreference: () => {
            return localStorage.getItem(LIGHTMODE) === 'true';
        },
        isTesting: () => {
            //Set to "true" to enable testing features (like quick login buttons)
            return true;
        }
    },
    actions: {
        setLightmodePreference(lightMode) {
            localStorage.setItem(LIGHTMODE, lightMode);
        }
    },
});
