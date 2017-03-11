import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { HttpModule } from '@angular/http';
import { MaterialModule } from '@angular/material';

import { AppComponent } from './app.component';
import { CodeComponent } from './code/code.component';
import { CodeService } from './code/code.service';

@NgModule({
  declarations: [
    AppComponent,
    CodeComponent
  ],
  imports: [
    BrowserModule,
    FormsModule,
    HttpModule,
    MaterialModule.forRoot(),
  ],
  providers: [CodeService],
  bootstrap: [AppComponent, CodeComponent]
})
export class AppModule { }
