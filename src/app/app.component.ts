import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { RouterOutlet } from '@angular/router';
import { invoke } from "@tauri-apps/api/core";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  helpMessage = "";

  help(event: MouseEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke<string>("help", { name }).then((text) => {
      this.helpMessage = text;
    });
  }

  click1() {
    
    invoke("click1").then()
  }

  click2() {
    invoke("click2").then()
  }

  click3() {
    invoke("click3").then()
  }
}
