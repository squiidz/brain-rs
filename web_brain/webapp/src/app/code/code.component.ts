import { Component, OnInit } from '@angular/core';

import { CodeService } from './code.service';
import { Code } from './code';
import { Output } from './output';

@Component({
  selector: 'app-code',
  templateUrl: './code.component.html',
  styleUrls: ['./code.component.css']
})

export class CodeComponent implements OnInit {
  code: string = "";
  args: string = "";
  output: Output = {output: "", length: 0};

  constructor(private codeService: CodeService) { }

  ngOnInit() {}

  sendCode() {
    let code = new Code(this.code, this.args);
    this.codeService.postCode(code).
      subscribe(
        out => {
          this.output = out;
          console.log(this.output); 
        },
        err => { console.log(err) });
  }

}
