import { invoke } from "@tauri-apps/api";
import { ShareComponent } from "./share.component";
import { Config } from "../types/config";

export default class ShareServiceAdapter {
    vm: ShareComponent;
    constructor(vm: ShareComponent) {
        this.vm = vm;
    }
    async initializeData() {
        this.vm.isLoading = true;
        this.vm.currentDirectory = await this.getHomeDirectory();
        this.vm.directoryNavigationStack.push(this.vm.currentDirectory);
        let result = await Promise.all([this.getCurrentDirectory(), this.getConfig()]);
        [this.vm.directoryData, this.vm.config] = result;
        this.vm.config_original = JSON.parse(JSON.stringify(this.vm.config));
        this.vm.isLoading = false;
    }

    async getCurrentDirectory(): Promise<Array<[string, boolean]>> {
        let result = await invoke<string>("generic_handler", {
            args: ["list_directory", this.vm.currentDirectory],
        }).then((response): Array<[string, boolean]> => JSON.parse(JSON.stringify(response)));
        result.sort((a: [string, boolean], b: [string, boolean]) => {
            return b[0].localeCompare(a[0]);
        });
        result.sort((a: [string, boolean], b: [string, boolean]) => {
            return a[1] ? -1 : 1;
        });
        return result;
    }
    async getHomeDirectory(): Promise<string> {
        let result: string = await invoke<string>("generic_handler", {
            args: ["get_home_directory"],
        });
        return result;
    }
    async isPathValid(path: string): Promise<boolean> {
        return await invoke<boolean>("generic_handler", {
            args: ["is_path_valid", path],
        });
    }
    async applyShare(): Promise<void> {
        let result = await invoke<boolean | string>("generic_handler", {
            args: ["set_config", JSON.stringify(this.vm.config)],
        });
        result == true ? alert("updated successfully") : alert("please try again");
        result == true ? null : console.log(result);
        this.vm.config_original = this.vm.config;
    }

    async getConfig(): Promise<Config> {
        return await invoke<Config>("generic_handler", {
            args: ["get_config"],
        });
    }
}
