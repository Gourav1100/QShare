// modules
import { NgModule } from "@angular/core";
import { MatSidenavModule } from "@angular/material/sidenav";
import { MatCardModule } from "@angular/material/card";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { CommonModule } from "@angular/common";
import { MatListModule } from "@angular/material/list";
import { MatCheckboxModule } from "@angular/material/checkbox";
import { MatTooltipModule } from "@angular/material/tooltip";
import { MatInputModule } from "@angular/material/input";
import { MatFormFieldModule } from "@angular/material/form-field";
import { FormsModule } from "@angular/forms";
import { MatSnackBarModule} from '@angular/material/snack-bar';
// components
import { SpinnerComponent } from "./spinner/spinner.component";
import { NotificationComponent } from "./notification/notification.component";

@NgModule({
    declarations: [SpinnerComponent, NotificationComponent],
    imports: [
        MatSnackBarModule,
        MatSidenavModule,
        MatCardModule,
        MatProgressSpinnerModule,
        CommonModule,
        MatListModule,
        MatCheckboxModule,
        MatTooltipModule,
        MatInputModule,
        MatFormFieldModule,
        FormsModule,
    ],
    exports: [
        MatSnackBarModule,
        SpinnerComponent,
        MatSidenavModule,
        MatCardModule,
        MatProgressSpinnerModule,
        CommonModule,
        MatListModule,
        MatCheckboxModule,
        MatTooltipModule,
        MatInputModule,
        MatFormFieldModule,
        FormsModule,
    ],
})
export class Common {}
