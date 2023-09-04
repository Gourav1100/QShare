import { Component, OnInit, ViewContainerRef } from "@angular/core";
import { Router } from "@angular/router";
import { Events } from "./common/events.module";
import { MatSnackBar } from "@angular/material/snack-bar";
import { NotificationComponent } from "./common/notification/notification.component";
import { invoke } from "@tauri-apps/api";

@Component({
    selector: "app-root",
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.css"],
})
export class AppComponent implements OnInit {
    menuItemList: Array<{ title: string; route: string; key: string; icon: string }> = [
        { key: "share_file", title: "Share a file", route: "share", icon: "send" },
        { key: "share_history", title: "History", route: "history", icon: "manage_history" },
    ];
    currentPage: string = "share_file";
    eventHandler: Events | undefined;
    constructor(private router: Router, private snackBar: MatSnackBar) {}

    ngOnInit(): void {
        //Called after the constructor, initializing input properties, and the first call to ngOnChanges.
        //Add 'implements OnInit' to the class.
        this.router.navigate(["/share"]);
        this.eventHandler = new Events();
        this.eventHandler.initializeAppHandler(this);
    }

    showNotification(ip_address: string): void {
        let handler = this.snackBar.openFromComponent(NotificationComponent, {
            duration: 10000,
            verticalPosition: "bottom",
            horizontalPosition: "center",
            data: {
                message: `Access request - ${ip_address.toString()}`,
            },
        });
        handler.afterDismissed().subscribe(async (status) => {
            let response = await invoke<boolean>("generic_handler", {
                args: ["update_status", status.dismissedByAction.toString(), ip_address],
            });
        });
    }

    getCurrentPageTitle(): string {
        let titleObject = this.menuItemList.find((menuItem) => menuItem.key == this.currentPage);
        return titleObject ? titleObject.title : "";
    }

    handlePageChange(menuItem: { title: string; route: string; key: string; icon: string }): void {
        this.currentPage = menuItem.key;
        this.router.navigate(["/" + menuItem.route]);
    }
}
