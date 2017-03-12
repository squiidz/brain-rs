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
  haveInput: boolean = false;
  bytecode: boolean = false;
  output: Output = {output: "", bytecode: "", length: 0, error: ""};

  constructor(private codeService: CodeService) { }

  ngOnInit() {}

  sendCode() {
    let code = new Code(this.code, this.args);
    this.codeService.postCode(code).
      subscribe(
        out => { this.output = out },
        err => { console.log(err) });
  }

  containsInput() {
    this.haveInput = this.code.includes(",");
  }

  toggleBytecode() {
    this.bytecode = this.bytecode ? false : true;
  }

}
