import { NgModule } from "@angular/core";
import { CommonModule } from "@angular/common";
import { listen } from "@tauri-apps/api/event";
import { AppComponent } from "../app.component";
import { NotificationComponent } from "./notification/notification.component";

@NgModule({
    declarations: [],
    imports: [CommonModule],
    exports: [],
})
export class Events {
    appHandler: AppComponent | undefined = undefined;
    // event handlers
    aboutEvent(event: any): void {
        console.log(`Use this to find the file: ${event.payload}`);
        alert("Native about button was clicked");
    }
    helpEvent(event: any): void {
        console.log(`Use this to find the file: ${event.payload}`);
        alert("Native help button was clicked");
    }
    openEvent(event: any): void {
        window.location.href = "/share";
    }
    // add events listeners
    constructor() {
        listen<string>("about", this.aboutEvent);
        listen<string>("authorize", (event) => {
            this.appHandler?.showNotification(event.payload);
        });
        listen<string>("share", this.openEvent);
        listen<string>("help", this.helpEvent);
    }
    initializeAppHandler(appComponent: AppComponent) {
        this.appHandler = appComponent;
    }
}
