import { Injectable } from '@angular/core';
import { Response, Headers, RequestOptions, Http } from '@angular/http';
import { Observable } from 'rxjs/Rx';
import { environment } from '../../environments/environment';

import 'rxjs/add/operator/map';

import { Code } from './code';
import { Output } from './output';

const apiUrl = `${environment.apiUrl}/run`;

@Injectable()
export class CodeService {

  constructor(private http: Http) {  }

  postCode(code: Code): Observable<Output> {
    let headers = new Headers({ 'Content-Type': 'application/json' });
    let options = new RequestOptions({ headers: headers });

    return this.http.post(apiUrl, {code: code.code, args: code.args}, options)
      .map((res:Response) => res.json()) 
      .catch((error:any) => Observable.throw(error.json().error || 'Server error'));
  }

//   addRemoteUser(user: User): Observable<User> {
//     let headers = new Headers({ 'Content-Type': 'application/json' });
//     let options = new RequestOptions({ headers: headers });

//     return this.http.post(`${usersUrl}/new`, {name: user.name, age: user.age}, options)
//       .map((res:Response) => res.json())
//       .catch((error:any) => Observable.throw(error.json().error || 'Server error'));
//   }

//   deleteUser(id: number): Observable<User> {
//     let options = new RequestOptions();
//     return this.http.delete(`${usersUrl}/${id}`, options)
//       .map((res:Response) => res.json())
//       .catch((error:any) => Observable.throw(error.json().error || 'Server error'));
//   }

}