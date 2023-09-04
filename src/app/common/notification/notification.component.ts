import { Component, Inject } from "@angular/core";
import { MAT_SNACK_BAR_DATA, MatSnackBarRef } from "@angular/material/snack-bar";

@Component({
    selector: "app-notification",
    templateUrl: "./notification.component.html",
    styleUrls: ["./notification.component.css"],
})
export class NotificationComponent {
    constructor(
        public snackBarRef: MatSnackBarRef<NotificationComponent>,
        @Inject(MAT_SNACK_BAR_DATA) public data: any
    ) {}

    updateRequestStatus(status: boolean) {
        status ? this.snackBarRef.dismissWithAction() : this.snackBarRef.dismiss();
    }
}
