import { Component, OnInit } from "@angular/core";
import ShareServiceAdapter from "./share.service.adapter";
import { Config } from "../types/config";

@Component({
    selector: "app-share",
    templateUrl: "./share.component.html",
    styleUrls: ["./share.component.css"],
})
export class ShareComponent implements OnInit {
    isLoading: boolean = true;
    directoryData: Array<[string, boolean]> = [];
    directoryNavigationStack: Array<string> = [];
    config: Config = {
        shared: {},
        approved: {},
    };
    config_original: Config = {
        shared: {},
        approved: {},
    };
    currentDirectory: string = "";
    serviceAdapter!: ShareServiceAdapter;
    isHiddenVisible: boolean = false;
    ngOnInit(): void {
        this.serviceAdapter = new ShareServiceAdapter(this);
        this.serviceAdapter.initializeData();
    }

    async toggleShare(index: number): Promise<void> {
        if (!Object.keys(this.config.shared).includes(this.directoryData[index][0])) {
            this.config.shared[this.directoryData[index][0]] = this.directoryData[index][1] ? 1 : 2;
        } else {
            delete this.config.shared[this.directoryData[index][0]];
        }
    }

    async navigateDirectory(directory: string): Promise<void> {
        this.isLoading = true;
        this.currentDirectory = directory;
        this.directoryNavigationStack.push(directory);
        let result = await this.serviceAdapter.getCurrentDirectory();
        this.config = JSON.parse(JSON.stringify(this.config_original));
        this.directoryData = result;
        this.isLoading = false;
    }
    async popDirectory(): Promise<void> {
        this.isLoading = true;
        this.directoryNavigationStack.pop();
        this.currentDirectory = this.directoryNavigationStack.at(-1)!;
        let result = await this.serviceAdapter.getCurrentDirectory();
        this.config = JSON.parse(JSON.stringify(this.config_original));
        this.directoryData = result;
        this.isLoading = false;
    }

    processTitle(title: string): string | undefined {
        let tokens = title.split("/");
        return tokens.at(-1)?.toString();
    }

    async pushCustomPath(event: any): Promise<void> {
        this.isLoading = true;
        event.preventDefault();
        event.target["path"].blur();
        if (await this.serviceAdapter.isPathValid(event.target["path"].value)) {
            this.navigateDirectory(event.target["path"].value);
        } else {
            event.target["path"].value = this.directoryNavigationStack.at(-1);
            this.isLoading = false;
            alert("Invalid path");
        }
    }

    checkConfig(): boolean {
        let keys_config = Object.keys(this.config.shared);
        let keys_config_original = Object.keys(this.config_original.shared);
        keys_config.sort((a, b) => a.localeCompare(b));
        keys_config_original.sort((a, b) => a.localeCompare(b));
        return JSON.stringify(keys_config) == JSON.stringify(keys_config_original);
    }
}
